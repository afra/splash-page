import ToriiAuthenticator from 'ember-simple-auth/authenticators/torii';
import { inject as service } from '@ember/service';
import fetch from 'fetch';

export default ToriiAuthenticator.extend({
  torii: service(),

  async authenticate() {
    const data = await this._super(...arguments);

    const res = await fetch('/api/v1/oauth', {
      method: 'POST',
      data: data.authorizationCode,
    });

    const json = await res.json();
    debugger;
  },
});
