const path = require('path');
module.exports = {
  entry: {
    fd_laplace: "./fd_laplace.js",
    mol_diffusion: "./mol_diffusion.js",
    linear_regression: "./linear_regression.js"
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "[name].bundle.js"
  },
  mode: "development",
  module: {
    rules: [
      {
        test: /\.js$/,
        use: [
          'ify-loader',
          'transform-loader?plotly.js/tasks/compress_attributes.js',
          ]
      },
    ]
  }
};
