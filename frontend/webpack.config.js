/* eslint-disable import/no-extraneous-dependencies */
const path = require("path");
const MonacoWebpackPlugin = require("monaco-editor-webpack-plugin");

module.exports = {
  entry: path.join(__dirname, "src", "main.tsx"),
  output: {
    path: path.join(__dirname, "dist"),
    filename: "main.js"
  },
  module: {
    rules: [
      {
        test: /\.tsx?/,
        use: "ts-loader"
      },
      {
        test: /\.css$/,
        use: ["style-loader", "css-loader"]
      },
      {
        test: /\.ttf$/,
        use: ["file-loader"]
      }
    ]
  },
  devtool: "inline-source-map",
  resolve: {
    extensions: [".js", ".ts", ".jsx", ".tsx"]
  },
  plugins: [new MonacoWebpackPlugin()]
};
