// Copyright 2020 The Model Authors. All rights reserved.
// Use of this source code is governed by the Apache License,
// Version 2.0, that can be found in the LICENSE file.

use std::collections::{BTreeSet, HashMap, HashSet};

#[cfg(test)]
use crate::ast::Loc;
use crate::ast::{parse_equation as parse_single_equation, Ast, Expr, Visitor};
use crate::builtins::is_builtin_fn;
use crate::builtins_visitor::instantiate_implicit_modules;
use crate::common::{DimensionName, EquationError, EquationResult, Ident, VariableError};
use crate::datamodel::Dimension;
use crate::units::parse_units;
use crate::{datamodel, eqn_err, units, ErrorCode};

#[derive(Clone, PartialEq, Debug)]
pub struct Table {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    x_range: datamodel::GraphicalFunctionScale,
    y_range: datamodel::GraphicalFunctionScale,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ModuleInput {
    // the Variable identifier in the current model we will use for input
    pub src: Ident,
    // the Variable identifier in the module's model we will override
    pub dst: Ident,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Variable {
    Stock {
        ident: Ident,
        ast: Option<Ast>,
        eqn: Option<datamodel::Equation>,
        units: Option<datamodel::UnitMap>,
        inflows: Vec<Ident>,
        outflows: Vec<Ident>,
        non_negative: bool,
        errors: Vec<VariableError>,
    },
    Var {
        ident: Ident,
        ast: Option<Ast>,
        eqn: Option<datamodel::Equation>,
        units: Option<datamodel::UnitMap>,
        table: Option<Table>,
        non_negative: bool,
        is_flow: bool,
        is_table_only: bool,
        errors: Vec<VariableError>,
    },
    Module {
        // the current spec has ident == model name
        ident: Ident,
        model_name: Ident,
        units: Option<datamodel::UnitMap>,
        inputs: Vec<ModuleInput>,
        errors: Vec<VariableError>,
    },
}

impl Variable {
    pub fn ident(&self) -> &str {
        match self {
            Variable::Stock { ident: name, .. } => name.as_str(),
            Variable::Var { ident: name, .. } => name.as_str(),
            Variable::Module { ident: name, .. } => name.as_str(),
        }
    }

    pub fn ast(&self) -> Option<&Ast> {
        match self {
            Variable::Stock { ast: Some(ast), .. } => Some(ast),
            Variable::Var { ast: Some(ast), .. } => Some(ast),
            _ => None,
        }
    }

    pub fn scalar_equation(&self) -> Option<&String> {
        match self {
            Variable::Stock {
                eqn: Some(datamodel::Equation::Scalar(s)),
                ..
            } => Some(s),
            Variable::Var {
                eqn: Some(datamodel::Equation::Scalar(s)),
                ..
            } => Some(s),
            _ => None,
        }
    }

    pub fn get_dimensions(&self) -> Option<&[Dimension]> {
        match self {
            Variable::Stock {
                ast: Some(Ast::Arrayed(dims, _)),
                ..
            } => Some(dims),
            Variable::Stock {
                ast: Some(Ast::ApplyToAll(dims, _)),
                ..
            } => Some(dims),
            Variable::Var {
                ast: Some(Ast::Arrayed(dims, _)),
                ..
            } => Some(dims),
            Variable::Var {
                ast: Some(Ast::ApplyToAll(dims, _)),
                ..
            } => Some(dims),
            _ => None,
        }
    }

    pub fn is_stock(&self) -> bool {
        matches!(self, Variable::Stock { .. })
    }

    pub fn is_module(&self) -> bool {
        matches!(self, Variable::Module { .. })
    }

    pub fn errors(
        &self,
        filter: fn(&VariableError) -> Option<EquationError>,
    ) -> Option<Vec<EquationError>> {
        let errors = match self {
            Variable::Stock { errors, .. } => errors,
            Variable::Var { errors, .. } => errors,
            Variable::Module { errors, .. } => errors,
        };

        let errors: Vec<_> = errors.iter().flat_map(filter).collect();

        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }

    pub fn equation_errors(&self) -> Option<Vec<EquationError>> {
        self.errors(|err| match err {
            VariableError::EquationError(err) => Some(err.clone()),
            VariableError::UnitError(_) => None,
        })
    }

    pub fn unit_errors(&self) -> Option<Vec<EquationError>> {
        self.errors(|err| match err {
            VariableError::EquationError(_) => None,
            VariableError::UnitError(err) => Some(err.clone()),
        })
    }

    pub fn push_error(&mut self, err: EquationError) {
        let err = VariableError::EquationError(err);
        match self {
            Variable::Stock { errors, .. } => errors.push(err),
            Variable::Var { errors, .. } => errors.push(err),
            Variable::Module { errors, .. } => errors.push(err),
        }
    }

    pub fn table(&self) -> Option<&Table> {
        match self {
            Variable::Stock { .. } => None,
            Variable::Var { table, .. } => table.as_ref(),
            Variable::Module { .. } => None,
        }
    }

    pub fn units(&self) -> Option<&datamodel::UnitMap> {
        match self {
            Variable::Stock { units, .. } => units.as_ref(),
            Variable::Var { units, .. } => units.as_ref(),
            Variable::Module { units, .. } => units.as_ref(),
        }
    }
}

#[allow(clippy::unnecessary_wraps)]
fn parse_table(gf: &Option<datamodel::GraphicalFunction>) -> EquationResult<Option<Table>> {
    if gf.is_none() {
        return Ok(None);
    }
    let gf = gf.as_ref().unwrap();

    let x: Vec<f64> = match &gf.x_points {
        Some(x_points) => x_points.clone(),
        None => {
            let x_min = gf.x_scale.min;
            let x_max = gf.x_scale.max;
            let size = gf.y_points.len() as f64;
            gf.y_points
                .iter()
                .enumerate()
                .map(|(i, _)| ((i as f64) / (size - 1.0)) * (x_max - x_min) + x_min)
                .collect()
        }
    };

    Ok(Some(Table {
        x,
        y: gf.y_points.clone(),
        x_range: gf.x_scale.clone(),
        y_range: gf.y_scale.clone(),
    }))
}

fn get_dimensions(
    dimensions: &[Dimension],
    names: &[DimensionName],
) -> Result<Vec<datamodel::Dimension>, EquationError> {
    names
        .iter()
        .map(|name| -> Result<datamodel::Dimension, EquationError> {
            for dim in dimensions {
                if dim.name == *name {
                    return Ok(dim.clone());
                }
            }
            eqn_err!(BadDimensionName, 0, 0)
        })
        .collect()
}

fn parse_equation(
    eqn: &datamodel::Equation,
    dimensions: &[Dimension],
) -> (Option<Ast>, Vec<EquationError>) {
    match eqn {
        datamodel::Equation::Scalar(eqn) => {
            match parse_single_equation(eqn).map(|eqn| eqn.map(Ast::Scalar)) {
                Ok(expr) => (expr, vec![]),
                Err(errors) => (None, errors),
            }
        }
        datamodel::Equation::ApplyToAll(dimension_names, eqn) => {
            let (ast, mut errors) = match parse_single_equation(eqn) {
                Ok(expr) => (expr, vec![]),
                Err(errors) => (None, errors),
            };
            match get_dimensions(dimensions, dimension_names) {
                Ok(dims) => (ast.map(|ast| Ast::ApplyToAll(dims, ast)), errors),
                Err(err) => {
                    errors.push(err);
                    (None, errors)
                }
            }
        }
        datamodel::Equation::Arrayed(dimension_names, elements) => {
            let mut errors: Vec<EquationError> = vec![];
            let elements: Vec<_> = elements
                .iter()
                .map(|(subscript, equation)| {
                    let (ast, single_errors) = match parse_single_equation(equation) {
                        Ok(expr) => (expr, vec![]),
                        Err(errors) => (None, errors),
                    };
                    errors.extend(single_errors);
                    (subscript.clone(), ast)
                })
                .filter(|(_, ast)| ast.is_some())
                .map(|(subscript, ast)| (subscript, ast.unwrap()))
                .collect();

            match get_dimensions(dimensions, dimension_names) {
                Ok(dims) => (
                    Some(Ast::Arrayed(dims, elements.iter().cloned().collect())),
                    errors,
                ),
                Err(err) => {
                    errors.push(err);
                    (None, errors)
                }
            }
        }
    }
}

pub fn parse_var(
    models: &HashMap<String, HashMap<Ident, &datamodel::Variable>>,
    model_name: &str,
    dimensions: &[Dimension],
    v: &datamodel::Variable,
    implicit_vars: &mut Vec<datamodel::Variable>,
    units_ctx: &units::Context,
) -> Variable {
    let mut parse_and_lower_eqn =
        |ident: &str, eqn: &datamodel::Equation| -> (Option<Ast>, Vec<EquationError>) {
            let (ast, mut errors) = parse_equation(eqn, dimensions);
            let ast = match ast {
                Some(ast) => match instantiate_implicit_modules(ident, ast) {
                    Ok((ast, mut new_vars)) => {
                        implicit_vars.append(&mut new_vars);
                        Some(ast)
                    }
                    Err(err) => {
                        errors.push(err);
                        None
                    }
                },
                None => {
                    if errors.is_empty() && !v.can_be_module_input() {
                        errors.push(EquationError {
                            start: 0,
                            end: 0,
                            code: ErrorCode::EmptyEquation,
                        })
                    }
                    None
                }
            };

            (ast, errors)
        };
    match v {
        datamodel::Variable::Stock(v) => {
            let ident = v.ident.clone();
            let (ast, errors) = parse_and_lower_eqn(&ident, &v.equation);
            let mut errors: Vec<VariableError> = errors
                .into_iter()
                .map(VariableError::EquationError)
                .collect();
            let units = match parse_units(units_ctx, v.units.as_ref()) {
                Ok(units) => units,
                Err(unit_errors) => {
                    for err in unit_errors.into_iter() {
                        errors.push(VariableError::UnitError(err));
                    }
                    None
                }
            };
            Variable::Stock {
                ident,
                ast,
                eqn: Some(v.equation.clone()),
                units,
                inflows: v.inflows.clone(),
                outflows: v.outflows.clone(),
                non_negative: v.non_negative,
                errors,
            }
        }
        datamodel::Variable::Flow(v) => {
            let ident = v.ident.clone();
            let (ast, errors) = parse_and_lower_eqn(&ident, &v.equation);
            let mut errors: Vec<VariableError> = errors
                .into_iter()
                .map(VariableError::EquationError)
                .collect();
            let units = match parse_units(units_ctx, v.units.as_ref()) {
                Ok(units) => units,
                Err(unit_errors) => {
                    for err in unit_errors.into_iter() {
                        errors.push(VariableError::UnitError(err));
                    }
                    None
                }
            };
            let table = match parse_table(&v.gf) {
                Ok(table) => table,
                Err(err) => {
                    // TODO: should have a TableError variant
                    errors.push(VariableError::EquationError(err));
                    None
                }
            };
            Variable::Var {
                ident,
                ast,
                eqn: Some(v.equation.clone()),
                units,
                table,
                is_flow: true,
                is_table_only: false,
                non_negative: v.non_negative,
                errors,
            }
        }
        datamodel::Variable::Aux(v) => {
            let ident = v.ident.clone();
            let (ast, errors) = parse_and_lower_eqn(&ident, &v.equation);
            let mut errors: Vec<VariableError> = errors
                .into_iter()
                .map(VariableError::EquationError)
                .collect();
            let units = match parse_units(units_ctx, v.units.as_ref()) {
                Ok(units) => units,
                Err(unit_errors) => {
                    for err in unit_errors.into_iter() {
                        errors.push(VariableError::UnitError(err));
                    }
                    None
                }
            };
            let table = match parse_table(&v.gf) {
                Ok(table) => table,
                Err(err) => {
                    // TODO: should have TableError variant
                    errors.push(VariableError::EquationError(err));
                    None
                }
            };
            Variable::Var {
                ident,
                ast,
                eqn: Some(v.equation.clone()),
                units,
                table,
                is_flow: false,
                is_table_only: false,
                non_negative: false,
                errors,
            }
        }
        datamodel::Variable::Module(v) => {
            let ident = v.ident.clone();
            let inputs: Vec<EquationResult<Option<ModuleInput>>> = v
                .references
                .iter()
                .map(|mi| {
                    crate::model::resolve_module_input(models, model_name, &ident, &mi.src, &mi.dst)
                })
                .collect();
            let (inputs, errors): (Vec<_>, Vec<_>) =
                inputs.into_iter().partition(EquationResult::is_ok);
            let inputs: Vec<ModuleInput> = inputs.into_iter().flat_map(|i| i.unwrap()).collect();
            let mut errors: Vec<VariableError> = errors
                .into_iter()
                .map(|e| e.unwrap_err())
                .map(VariableError::EquationError)
                .collect();
            let units = match parse_units(units_ctx, v.units.as_ref()) {
                Ok(units) => units,
                Err(unit_errors) => {
                    for err in unit_errors.into_iter() {
                        errors.push(VariableError::UnitError(err));
                    }
                    None
                }
            };

            Variable::Module {
                model_name: v.model_name.clone(),
                ident,
                units,
                inputs,
                errors,
            }
        }
    }
}

struct IdentifierSetVisitor<'a> {
    identifiers: HashSet<Ident>,
    dimensions: &'a [Dimension],
    module_inputs: Option<&'a BTreeSet<Ident>>,
}

impl<'a> Visitor<()> for IdentifierSetVisitor<'a> {
    fn walk(&mut self, e: &Expr) {
        match e {
            Expr::Const(_, _, _) => (),
            Expr::Var(id, _) => {
                if !is_builtin_fn(id) {
                    self.identifiers.insert(id.clone());
                }
            }
            Expr::App(func, args, _) => {
                if !is_builtin_fn(func) {
                    self.identifiers.insert(func.clone());
                }
                for arg in args.iter() {
                    self.walk(arg);
                }
            }
            Expr::Subscript(id, args, _) => {
                if !is_builtin_fn(id) {
                    self.identifiers.insert(id.clone());
                }
                for arg in args.iter() {
                    if let Expr::Var(arg_ident, _) = arg {
                        let mut is_subscript_or_dimension = false;
                        // TODO: this should be optimized
                        for dim in self.dimensions.iter() {
                            if arg_ident == &dim.name {
                                is_subscript_or_dimension = true;
                            }
                            for element_name in dim.elements.iter() {
                                // subscript names aren't dependencies
                                if arg_ident == element_name {
                                    is_subscript_or_dimension = true;
                                }
                            }
                        }
                        if !is_subscript_or_dimension {
                            self.walk(arg);
                        }
                    } else {
                        self.walk(arg)
                    }
                }
            }
            Expr::Op2(_, l, r, _) => {
                self.walk(l);
                self.walk(r);
            }
            Expr::Op1(_, l, _) => {
                self.walk(l);
            }
            Expr::If(cond, t, f, _) => {
                if let Some(module_inputs) = self.module_inputs {
                    if let Expr::App(builtin_id, args, _) = cond.as_ref() {
                        if builtin_id == "ismoduleinput" && args.len() == 1 {
                            if let Expr::Var(ident, _) = &args[0] {
                                if module_inputs.contains(ident) {
                                    self.walk(t);
                                } else {
                                    self.walk(f);
                                }
                                return;
                            }
                        }
                    }
                }

                self.walk(cond);
                self.walk(t);
                self.walk(f);
            }
        }
    }
}

pub fn identifier_set(
    ast: &Ast,
    dimensions: &[Dimension],
    module_inputs: Option<&BTreeSet<Ident>>,
) -> HashSet<Ident> {
    let mut id_visitor = IdentifierSetVisitor {
        identifiers: HashSet::new(),
        dimensions,
        module_inputs,
    };
    match ast {
        Ast::Scalar(ast) => id_visitor.walk(ast),
        Ast::ApplyToAll(_, ast) => id_visitor.walk(ast),
        Ast::Arrayed(_, elements) => {
            for ast in elements.values() {
                id_visitor.walk(ast);
            }
        }
    };
    id_visitor.identifiers
}

#[test]
fn test_identifier_sets() {
    let cases: &[(&str, &[&str])] = &[
        ("if isModuleInput(input) then b else c", &["b"]),
        ("if a then b else c", &["a", "b", "c"]),
        ("a(1, b, c)", &["a", "b", "c"]),
        ("-(a)", &["a"]),
        ("if a = 1 then -c else c(1, d, b)", &["a", "b", "c", "d"]),
        ("if a.d then b else c", &["a·d", "b", "c"]),
        ("if \"a.d\" then b else c", &["a.d", "b", "c"]),
        ("g[foo]", &["g"]),
    ];

    let dimensions: &[Dimension] = &[Dimension {
        name: "dim1".to_string(),
        elements: vec!["foo".to_owned()],
    }];

    let module_inputs: &[ModuleInput] = &[ModuleInput {
        src: "whatever".to_string(),
        dst: "input".to_string(),
    }];

    for (eqn, id_list) in cases.iter() {
        let (ast, err) = parse_equation(&datamodel::Equation::Scalar((*eqn).to_owned()), &[]);
        assert_eq!(err.len(), 0);
        assert!(ast.is_some());
        let ast = ast.unwrap();
        let id_set_expected: HashSet<Ident> = id_list.into_iter().map(|s| s.to_string()).collect();
        let module_input_names = module_inputs.iter().map(|mi| mi.dst.clone()).collect();
        let id_set_test = identifier_set(&ast, &dimensions, Some(&module_input_names));
        assert_eq!(id_set_expected, id_set_test);
    }
}

#[test]
fn test_tables() {
    use crate::common::canonicalize;
    let input = datamodel::Variable::Aux(datamodel::Aux {
        ident: canonicalize("lookup function table"),
        equation: datamodel::Equation::Scalar("0".to_string()),
        documentation: "".to_string(),
        units: None,
        gf: Some(datamodel::GraphicalFunction {
            kind: datamodel::GraphicalFunctionKind::Continuous,
            x_scale: datamodel::GraphicalFunctionScale {
                min: 0.0,
                max: 45.0,
            },
            y_scale: datamodel::GraphicalFunctionScale {
                min: -1.0,
                max: 1.0,
            },
            x_points: None,
            y_points: vec![0.0, 0.0, 1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 0.0],
        }),
        can_be_module_input: false,
        visibility: datamodel::Visibility::Private,
    });

    let expected = Variable::Var {
        ident: "lookup_function_table".to_string(),
        ast: Some(Ast::Scalar(Expr::Const(
            "0".to_string(),
            0.0,
            Loc::new(0, 1),
        ))),
        eqn: Some(datamodel::Equation::Scalar("0".to_string())),
        units: None,
        table: Some(Table {
            x: vec![0.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0, 35.0, 40.0, 45.0],
            y: vec![0.0, 0.0, 1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 0.0],
            x_range: datamodel::GraphicalFunctionScale {
                min: 0.0,
                max: 45.0,
            },
            y_range: datamodel::GraphicalFunctionScale {
                min: -1.0,
                max: 1.0,
            },
        }),
        non_negative: false,
        is_flow: false,
        is_table_only: false,
        errors: vec![],
    };

    if let Variable::Var {
        table: Some(table), ..
    } = &expected
    {
        assert_eq!(table.x.len(), table.y.len());
    } else {
        assert!(false);
    }

    let mut implicit_vars: Vec<datamodel::Variable> = Vec::new();
    let unit_ctx = crate::units::Context::new(&[]).unwrap();
    let output = parse_var(
        &HashMap::new(),
        "main",
        &[],
        &input,
        &mut implicit_vars,
        &unit_ctx,
    );

    assert_eq!(expected, output);
}
