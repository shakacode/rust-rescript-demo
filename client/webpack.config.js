const webpack = require("webpack");
const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");

const isProduction = process.env.NODE_ENV === "production";

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
  devtool: "cheap-eval-source-map",
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
  module: {
    rules: [
      {
        test: /\.js$/,
        use: [
          { loader: "babel-loader" },
          {
            loader: "@linaria/webpack4-loader",
            options: {
              displayName: !isProduction,
              sourceMap: !isProduction,
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
