const path = require('path')
const CopyPlugin = require('copy-webpack-plugin')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const HtmlWebpackPlugin = require('html-webpack-plugin')

const dist = path.resolve(__dirname, 'dist')

module.exports = {
  mode: 'production',
  devtool: false,
  entry: {
    index: './js/index.js'
  },
  output: {
    path: dist,
    filename: '[name].js'
  },
  devServer: {
    contentBase: dist
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: [{ loader: 'style-loader' }, { loader: 'css-loader' }]
      }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: 'index.html'
    }),

    new CopyPlugin([path.resolve(__dirname, 'static')]),

    new WasmPackPlugin({
      crateDirectory: __dirname
      // extraArgs: '--out-name index'
    })
  ]
}
