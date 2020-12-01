const webpack = require("webpack");
const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");

module.exports = {
  mode: "development",
  context: process.cwd(),
  entry: {
    app: "./src/index.bs.js",
  },
  output: {
    path: path.resolve(process.cwd(), "build"),
    publicPath: "/",
    filename: "[name].js",
    chunkFilename: "[id].js",
  },
  devtool: "#cheap-module-eval-source-map",
  devServer: {
    contentBase: "./",
    historyApiFallback: true,
  },
  plugins: [
    new webpack.EnvironmentPlugin([
      "NODE_ENV",
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
  module: {
    rules: [
      {
        test: /\.js$/,
        use: [
          { loader: "babel-loader" },
          {
            loader: "@linaria/webpack4-loader",
            options: {
              displayName: true,
              sourceMap: true,
            },
          },
        ],
      },
      {
        test: /\.css$/,
        use: [
          "style-loader",
          {
            loader: "css-loader",
            options: { modules: "global" },
          },
        ],
      },
    ],
  },
};
