<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>Todo</title>
    <link rel="stylesheet" href="http://todomvc.com/examples/react/node_modules/todomvc-common/base.css">
    <link rel="stylesheet" href="http://todomvc.com/examples/react/node_modules/todomvc-app-css/index.css">
    <style type="text/css">
    label a {
      color: #4d4d4d;
      text-decoration: none;
    }
    </style>
    <script>
     document.addEventListener('click', function clickHandler(e) {
       if (e.target && e.target.dataset && e.target.dataset.id) {
         window.location.href = '/' + e.target.dataset.action + '/' + e.target.dataset.id;
       }
     });
   </script>
  </head>
  <body>
    <section class="todoapp">
      <header class="header">
        <h1>Todos</h1>
        <form class="add-todo" action="/" method="/">
          <input class="new-todo" placeholder="What then?" name="todo" />
        </form>
      </header>

      <main class="main">
        <ul class="todo-list">
          {{#each todos}}
          {{#unless deleted}}
          {{#filter_todo}}
          <li {{#if completed}}class="completed"{{/if}} data-id="{{id}}">
            <div class="view">
              <input class="toggle" type="checkbox" {{#if completed}}checked="checked"{{/if}} data-id={{id}} data-action="toggle">
              <label><a href="/toggle/{{id}}">{{description}}</a></label>
              <button class="destroy" data-id={{id}} data-action="remove"></button>
            </div>
          </li>
          {{/filter_todo}}
          {{/unless}}
          {{/each}}
        </ul>
      </main>

      <footer class="footer">
        <span class="todo-count">
          <strong>0 items left</strong>
        </span>
        <ul class="filters">
          <li><a href="/show/all">All</a></li>
          <li><a href="/show/active">Active</a></li>
          <li><a href="/show/completed">Completed</a></li>
        </ul>
      </footer>
    </section>
  </body>
</html>
