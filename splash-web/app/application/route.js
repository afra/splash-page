import Route from '@ember/routing/route';
import ApplicationRouteMixin from 'ember-simple-auth/mixins/application-route-mixin';

export default Route.extend(ApplicationRouteMixin, {
  async model() {
    const status = await fetch('/api/v1/open');
    const open = await status.json();

    return {open};
  }
});
