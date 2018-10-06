import Component from '@ember/component';
import { computed } from '@ember/object';
import fetch from 'fetch';
import remark from 'remark';
import html from 'remark-html';

export default Component.extend({
  async didReceiveAttrs() {
    const res = await fetch(`content/${this.name}.md`);
    const md = await res.text();

    const content = await remark()
      .use(html)
      .process(md);

    this.set('content', content);
  }
});
