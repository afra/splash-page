import ApplicationAction from '../application';
import SpaceEvent from '../../models/space-event';

export default class DestroySpaceEvent extends ApplicationAction {

  async respond({ params }) {
    let spaceEvent = await SpaceEvent.find(params.id);
    await spaceEvent.delete();
    this.render(204);
  }

}
