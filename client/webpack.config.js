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
  },
  plugins: [
    new CleanWebpackPlugin({
      cleanStaleWebpackAssets: false,
    }),
    new HtmlWebpackPlugin({
      template: "src/index.html",
    }),
  ],
};
