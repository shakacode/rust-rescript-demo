const webpack = require("webpack");
const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");

module.exports = {
  mode: "development",
  entry: {
    app: "./src/index.bs.js",
  },
  output: {
    filename: "[name].bundle.js",
    path: path.resolve(__dirname, "build"),
    publicPath: "/",
  },
  devtool: "inline-source-map",
  devServer: {
    contentBase: "./build",
    historyApiFallback: true,
  },
  plugins: [
    new webpack.EnvironmentPlugin([
      "API_HOST",
      "API_PORT",
      "API_GRAPHQL_PATH",
    ]),
    new CleanWebpackPlugin({
      cleanStaleWebpackAssets: false,
    }),
    new HtmlWebpackPlugin({
      template: "src/index.html",
    }),
  ],
};
