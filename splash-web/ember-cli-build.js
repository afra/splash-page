/* eslint-env node */
'use strict';

const EmberApp = require('ember-cli/lib/broccoli/ember-app');

const Funnel = require('broccoli-funnel');
const MergeTrees = require('broccoli-merge-trees');

module.exports = function(defaults) {
  const app = new EmberApp(defaults, {
    // Add options here
  });

  const sourceSansPro = new Funnel('node_modules/@typopro/', {
    destDir: 'assets/typopro/'
  });

	return new MergeTrees([app.toTree(), sourceSansPro]);
};
