import CleanWebpackPlugin from "clean-webpack-plugin";
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
    // Currently we clean and copy favicon files to the build folder on every build
    // This can seem a bit inefficient and a waste of resources, but I want to be able to keep favicon in src and saved in the repo
    new CleanWebpackPlugin({
      // Thank you https://github.com/webpack-contrib/copy-webpack-plugin/issues/385#issuecomment-508914721
      cleanStaleWebpackAssets: false,
      cleanOnceBeforeBuildPatterns: ["**/*", "!media", "!media/*", "!stats-*.json"],
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