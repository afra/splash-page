import ApplicationAction from '../application';
import SpaceEvent from '../../models/space-event';

export default class ShowSpaceEvent extends ApplicationAction {

  async respond({ params }) {
    return await SpaceEvent.find(params.id);
  }

}
