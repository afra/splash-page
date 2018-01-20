import Controller from '@ember/controller';
import {computed} from '@ember/object';

export default Controller.extend({
  open: computed.alias('model.open'),
});
