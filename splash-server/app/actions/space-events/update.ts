import ApplicationAction from '../application';
import SpaceEvent from '../../models/space-event';

export default class UpdateSpaceEvent extends ApplicationAction {

  async respond({ params, body }) {
    let spaceEvent = await SpaceEvent.find(params.id);
    Object.assign(spaceEvent, body);
    return await spaceEvent.save();
  }

}
