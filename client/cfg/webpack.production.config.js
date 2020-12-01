const webpack = require("webpack");
const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const CssPlugin = require("mini-css-extract-plugin");
const OptimizeCssAssetsPlugin = require("optimize-css-assets-webpack-plugin");
const CompressionPlugin = require("compression-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");

module.exports = {
  mode: "production",
  context: process.cwd(),
  entry: {
    app: "./src/index.bs.js",
  },
  output: {
    path: path.resolve(process.cwd(), "build"),
    publicPath: "/",
    filename: "[name].[hash].js",
    chunkFilename: "[id].[hash].js",
  },
  devtool: false,
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
    new CssPlugin({
      filename: "[name].[hash].css",
      chunkFilename: "[id].[hash].css",
    }),
    new OptimizeCssAssetsPlugin({
      cssProcessor: require("cssnano"),
    }),
    new CompressionPlugin({
      deleteOriginalAssets: false,
    }),
  ],
  module: {
    rules: [
      {
        test: /\.js$/,
        use: [
          {
            loader: "babel-loader",
            options: { compact: true },
          },
          {
            loader: "@linaria/webpack4-loader",
            options: {
              displayName: false,
              sourceMap: false,
            },
          },
        ],
      },
      {
        test: /\.css$/,
        use: [
          CssPlugin.loader,
          {
            loader: "css-loader",
            options: { modules: "global" },
          },
        ],
      },
    ],
  },
};
