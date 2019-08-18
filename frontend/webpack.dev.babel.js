import common from "./webpack.common.babel.js";
import merge from "webpack-merge";

export default merge(common, {
    devServer: {
        contentBase: "./build"
    },

    devtool: "eval-source-map",

    mode: "development"
});