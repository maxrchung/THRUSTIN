import common from "./webpack.common.babel.js";
import CompressionPlugin from 'compression-webpack-plugin';
import merge from "webpack-merge";
import OptimizeCSSAssetsPlugin from "optimize-css-assets-webpack-plugin";
import RobotstxtPlugin from "robotstxt-webpack-plugin";
import SitemapPlugin from 'sitemap-webpack-plugin';
import TerserJSPlugin from "terser-webpack-plugin";

export default merge(common, {
    mode: "production",
    optimization: {
        minimizer: [
            new TerserJSPlugin({
                cache: true,
                parallel: true,
            }),
            new OptimizeCSSAssetsPlugin()
        ]
    },
    plugins: [
        new CompressionPlugin({
            cache: true,
            // Try and compress all files, probably better for NGINX serving this
            minRatio: 1
        }),
        new SitemapPlugin("https://THRUSTIN.rs", ["/"]),
        new RobotstxtPlugin({
            sitemap: "https://THRUSTIN.rs/sitemap.xml",
            host: "https://THRUSTIN.rs"
        })
    ],
});