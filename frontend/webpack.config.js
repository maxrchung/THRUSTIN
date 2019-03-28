const path = require("path");
module.exports = {
  entry: "./src/main.jsx",
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        loader: "babel-loader"
      }
    ]
  },
  resolve: { extensions: ["*", ".js", ".jsx"] },
  output: {
    path: path.resolve(__dirname, "./build/"),
    publicPath: "./build/",
    filename: "bundle.js"
  },
};