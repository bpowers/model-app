(globalThis.webpackChunksimlin_site=globalThis.webpackChunksimlin_site||[]).push([[103],{1133:(e,t,a)=>{"use strict";a.d(t,{Z:()=>E});var n=a(7378),l=a(8944),r=a(5318),s=a(3128),i=a(3298),o=a(8311),c=a(2152),m=a(7165),d=a(5561);const g="blogPostTitle_28zC",u="blogPostData_2ipU",p="blogPostDetailsFull_2LNT";const E=function(e){const t=function(){const{selectMessage:e}=(0,o.c2)();return t=>{const a=Math.ceil(t);return e(a,(0,s.I)({id:"theme.blog.post.readingTime.plurals",description:'Pluralized label for "{readingTime} min read". Use as much plural forms (separated by "|") as your language support (see https://www.unicode.org/cldr/cldr-aux/charts/34/supplemental/language_plural_rules.html)',message:"One min read|{readingTime} min read"},{readingTime:a}))}}(),{children:a,frontMatter:E,metadata:v,truncated:b,isBlogPostPage:h=!1}=e,{date:_,formattedDate:f,permalink:N,tags:Z,readingTime:k,title:T,editUrl:I}=v,{author:L,image:w,keywords:C}=E,P=E.author_url||E.authorURL,x=E.author_title||E.authorTitle,y=E.author_image_url||E.authorImageURL;return n.createElement(n.Fragment,null,n.createElement(m.Z,{keywords:C,image:w}),n.createElement("article",{className:h?void 0:"margin-bottom--xl"},(()=>{const e=h?"h1":"h2";return n.createElement("header",null,n.createElement(e,{className:g},h?T:n.createElement(i.Z,{to:N},T)),n.createElement("div",{className:(0,l.Z)(u,"margin-vert--md")},n.createElement("time",{dateTime:_},f),k&&n.createElement(n.Fragment,null," \xb7 ",t(k))),n.createElement("div",{className:"avatar margin-vert--md"},y&&n.createElement(i.Z,{className:"avatar__photo-link avatar__photo",href:P},n.createElement("img",{src:y,alt:L})),n.createElement("div",{className:"avatar__intro"},L&&n.createElement(n.Fragment,null,n.createElement("div",{className:"avatar__name"},n.createElement(i.Z,{href:P},L)),n.createElement("small",{className:"avatar__subtitle"},x)))))})(),n.createElement("div",{className:"markdown"},n.createElement(r.Zo,{components:c.Z},a)),(Z.length>0||b)&&n.createElement("footer",{className:(0,l.Z)("row docusaurus-mt-lg",{[p]:h})},Z.length>0&&n.createElement("div",{className:"col"},n.createElement("b",null,n.createElement(s.Z,{id:"theme.tags.tagsListLabel",description:"The label alongside a tag list"},"Tags:")),Z.map((({label:e,permalink:t})=>n.createElement(i.Z,{key:t,className:"margin-horiz--sm",to:t},e)))),h&&I&&n.createElement("div",{className:"col margin-top--sm"},n.createElement(d.Z,{editUrl:I})),!h&&b&&n.createElement("div",{className:"col text--right"},n.createElement(i.Z,{to:v.permalink,"aria-label":`Read more about ${T}`},n.createElement("b",null,n.createElement(s.Z,{id:"theme.blog.post.readMore",description:"The label used in blog post item excerpts to link to full blog posts"},"Read More")))))))}},1393:(e,t,a)=>{"use strict";a.r(t),a.d(t,{default:()=>g});var n=a(7378),l=a(5868),r=a(1133),s=a(3128),i=a(3298);const o=function(e){const{nextItem:t,prevItem:a}=e;return n.createElement("nav",{className:"pagination-nav docusaurus-mt-lg","aria-label":(0,s.I)({id:"theme.blog.post.paginator.navAriaLabel",message:"Blog post page navigation",description:"The ARIA label for the blog posts pagination"})},n.createElement("div",{className:"pagination-nav__item"},a&&n.createElement(i.Z,{className:"pagination-nav__link",to:a.permalink},n.createElement("div",{className:"pagination-nav__sublabel"},n.createElement(s.Z,{id:"theme.blog.post.paginator.newerPost",description:"The blog post button label to navigate to the newer/previous post"},"Newer Post")),n.createElement("div",{className:"pagination-nav__label"},"\xab ",a.title))),n.createElement("div",{className:"pagination-nav__item pagination-nav__item--next"},t&&n.createElement(i.Z,{className:"pagination-nav__link",to:t.permalink},n.createElement("div",{className:"pagination-nav__sublabel"},n.createElement(s.Z,{id:"theme.blog.post.paginator.olderPost",description:"The blog post button label to navigate to the older/next post"},"Older Post")),n.createElement("div",{className:"pagination-nav__label"},t.title," \xbb"))))};var c=a(5220),m=a(2079),d=a(8311);const g=function(e){const{content:t,sidebar:a}=e,{frontMatter:s,metadata:i}=t,{title:g,description:u,nextItem:p,prevItem:E}=i,{hide_table_of_contents:v}=s;return n.createElement(l.Z,{title:g,description:u,wrapperClassName:d.kM.wrapper.blogPages,pageClassName:d.kM.page.blogPostPage},t&&n.createElement("div",{className:"container margin-vert--lg"},n.createElement("div",{className:"row"},n.createElement("aside",{className:"col col--3"},n.createElement(c.Z,{sidebar:a})),n.createElement("main",{className:"col col--7"},n.createElement(r.Z,{frontMatter:s,metadata:i,isBlogPostPage:!0},n.createElement(t,null)),(p||E)&&n.createElement(o,{nextItem:p,prevItem:E})),!v&&t.toc&&n.createElement("div",{className:"col col--2"},n.createElement(m.Z,{toc:t.toc})))))}},5220:(e,t,a)=>{"use strict";a.d(t,{Z:()=>u});var n=a(7378),l=a(8944),r=a(3298);const s="sidebar_3pri",i="sidebarItemTitle_2iko",o="sidebarItemList_3aXd",c="sidebarItem_2HDj",m="sidebarItemLink_VIvG",d="sidebarItemLinkActive_34mL";var g=a(3128);function u({sidebar:e}){return 0===e.items.length?null:n.createElement("nav",{className:(0,l.Z)(s,"thin-scrollbar"),"aria-label":(0,g.I)({id:"theme.blog.sidebar.navAriaLabel",message:"Blog recent posts navigation",description:"The ARIA label for recent posts in the blog sidebar"})},n.createElement("div",{className:(0,l.Z)(i,"margin-bottom--md")},e.title),n.createElement("ul",{className:o},e.items.map((e=>n.createElement("li",{key:e.permalink,className:c},n.createElement(r.Z,{isNavLink:!0,to:e.permalink,className:m,activeClassName:d},e.title))))))}},5561:(e,t,a)=>{"use strict";a.d(t,{Z:()=>c});var n=a(7378),l=a(3128),r=a(9603),s=a(8944);const i="iconEdit_1CBY",o=({className:e,...t})=>n.createElement("svg",(0,r.Z)({fill:"currentColor",height:"20",width:"20",viewBox:"0 0 40 40",className:(0,s.Z)(i,e),"aria-hidden":"true"},t),n.createElement("g",null,n.createElement("path",{d:"m34.5 11.7l-3 3.1-6.3-6.3 3.1-3q0.5-0.5 1.2-0.5t1.1 0.5l3.9 3.9q0.5 0.4 0.5 1.1t-0.5 1.2z m-29.5 17.1l18.4-18.5 6.3 6.3-18.4 18.4h-6.3v-6.2z"})));function c({editUrl:e}){return n.createElement("a",{href:e,target:"_blank",rel:"noreferrer noopener"},n.createElement(o,null),n.createElement(l.Z,{id:"theme.common.editThisPage",description:"The link label to edit the current page"},"Edit this page"))}},2079:(e,t,a)=>{"use strict";a.d(t,{Z:()=>c});var n=a(7378),l=a(8944);const r=function(e,t,a){const[l,r]=(0,n.useState)(void 0);(0,n.useEffect)((()=>{function n(){const n=function(){const e=Array.from(document.getElementsByClassName("anchor")),t=e.find((e=>{const{top:t}=e.getBoundingClientRect();return t>=a}));if(t){if(t.getBoundingClientRect().top>=a){const a=e[e.indexOf(t)-1];return null!=a?a:t}return t}return e[e.length-1]}();if(n){let a=0,s=!1;const i=document.getElementsByClassName(e);for(;a<i.length&&!s;){const e=i[a],{href:o}=e,c=decodeURIComponent(o.substring(o.indexOf("#")+1));n.id===c&&(l&&l.classList.remove(t),e.classList.add(t),r(e),s=!0),a+=1}}}return document.addEventListener("scroll",n),document.addEventListener("resize",n),n(),()=>{document.removeEventListener("scroll",n),document.removeEventListener("resize",n)}}))},s="tableOfContents_3J2a",i="table-of-contents__link";function o({toc:e,isChild:t}){return e.length?n.createElement("ul",{className:t?"":"table-of-contents table-of-contents__left-border"},e.map((e=>n.createElement("li",{key:e.id},n.createElement("a",{href:`#${e.id}`,className:i,dangerouslySetInnerHTML:{__html:e.value}}),n.createElement(o,{isChild:!0,toc:e.children}))))):null}const c=function({toc:e}){return r(i,"table-of-contents__link--active",100),n.createElement("div",{className:(0,l.Z)(s,"thin-scrollbar")},n.createElement(o,{toc:e}))}}}]);