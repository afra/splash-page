import Route from '@ember/routing/route';

export default Route.extend({
  async model() {
    const status = await fetch('/api/v1/open');
    const open = await status.json();

    return {open};
  }
});
