/* eslint-disable import/no-extraneous-dependencies */
const path = require("path");

const DotEnv = require("dotenv-webpack");
const MonacoWebpackPlugin = require("monaco-editor-webpack-plugin");
const BundleAnalyzerPlugin = require("webpack-bundle-analyzer").BundleAnalyzerPlugin;

module.exports = {
  entry: path.join(__dirname, "src", "main.tsx"),
  output: {
    path: path.join(__dirname, "dist"),
    filename: "[name].bundle.js",
    chunkFilename: "[name].bundle.js",
    publicPath: "/"
  },
  module: {
    rules: [
      {
        test: /\.tsx?/,
        use: "ts-loader",
        exclude: /node_modules/
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
  plugins: [
    new DotEnv({
      path: path.join(__dirname, ".env"),
      systemvars: true
    }),
    new MonacoWebpackPlugin({
      languages: ["css", "html", "javascript", "json", "perl", "typescript"],
      features: [
        "!accessibilityHelp",
        "!codeAction",
        "!codelens",
        "!colorDetector",
        "!dnd",
        "!format",
        "!gotoError",
        "!gotoSymbol",
        "!parameterHints",
        "!referenceSearch",
        "!snippets",
        "!toggleHighContrast"
      ]
    }),
    new BundleAnalyzerPlugin()
  ]
};
