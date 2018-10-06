import Controller from '@ember/controller';
import {computed} from '@ember/object';
import { inject as service } from '@ember/service';

export default Controller.extend({
  session: service(),
  torii: service(),
  open: computed.alias('model.open'),

  actions: {
    login() {
      this.get('session').authenticate('authenticator:torii', 'github');
    },
  },
});
