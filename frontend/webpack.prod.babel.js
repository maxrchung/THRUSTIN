import common from "./webpack.common.babel.js";
import CompressionPlugin from 'compression-webpack-plugin';
import merge from "webpack-merge";
import OptimizeCSSAssetsPlugin from "optimize-css-assets-webpack-plugin";
import SitemapPlugin from 'sitemap-webpack-plugin';
import TerserJSPlugin from "terser-webpack-plugin";

export default merge(common, {
    mode: "production",
    optimization: {
        minimizer: [
          new TerserJSPlugin({}),
          new OptimizeCSSAssetsPlugin({})
        ]
    },
    plugins: [
        new CompressionPlugin({
            // Try and compress all files, probably better for NGINX serving this
            minRatio: 1 
        }),
        new SitemapPlugin("https://THRUSTIN.rs", ["/"])
    ],
});