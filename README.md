# CSS Markup Language (CSSML)

Transpiler for converting experimental `.cssml` into HTML & CSS.

> [!NOTE]
> CSSML is a hypothetical markup language which syntactially resembles and
> extends CSS by 'colocating' HTML generating capabilities within CSS.
> This is an experimental project and not intended for actual use.

## Idea

```css
/* index.cssml */
html () {
  head () {
    title (My site title) {
    }
  }
  body () {
    h1.headline (Welcome to my site) {
      font-size: 40px;
      font-weight: bold;
    }

    .headline {
      color: blue;
    }
  }
}
```

Generated source:

```html
<html>
  <head>
    <title>My site title</title>
  </head>
  <body>
    <h1 class="headline">Welcome to my site</h1>
  </body>
  <style>
    html body h1.headline {
      font-size: 40px;
      font-weight: bold;
    }

    html body .headline {
      color: blue;
    }
  </style>
</html>
```
