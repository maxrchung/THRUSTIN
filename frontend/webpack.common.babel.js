import CleanWebpackPlugin from "clean-webpack-plugin";
import CompressionPlugin from 'compression-webpack-plugin';
import CopyPlugin from "copy-webpack-plugin";
import HtmlWebpackPlugin from "html-webpack-plugin";
import MiniCssExtractPlugin from "mini-css-extract-plugin";
import path from "path";

export default {
  entry: "./src/main.jsx",
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        loader: "babel-loader"
      },
      {
        test: /\.scss$/,
        use: [
          MiniCssExtractPlugin.loader,
          "css-loader",
          "sass-loader"
        ]
      }
    ]
  },
  output: {
    path: path.resolve(__dirname, "./build/"),
    filename: "[name].[contenthash].js"
  },
  plugins: [
    new CleanWebpackPlugin({
      // Thank you https://github.com/webpack-contrib/copy-webpack-plugin/issues/385#issuecomment-508914721
      cleanStaleWebpackAssets: false,
    }),
    new MiniCssExtractPlugin({
      filename: "[name].[contenthash].css"
    }),
    new HtmlWebpackPlugin({
      template: "src/index.ejs"
    }),
    new CopyPlugin([
      { from: "src/favicon" },
    ])
  ],
  resolve: { extensions: ["*", ".js", ".jsx"] }
};