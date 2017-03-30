var { graphql, buildSchema } = require('graphql');

var schema = buildSchema(`
  type Query {
    hello: String,
    foo: String,
  }
`);

var root = {
  hello: () => 'Hello world!',
  foo: () => "this is foo",
};

graphql(schema, '{ hello, foo }', root)
  .then((response) => {
    console.log(response);
  });
