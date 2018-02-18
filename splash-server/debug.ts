import {
  createConnection,
  getManager,
} from 'typeorm';

import { SpaceEvent } from './app/models/space-event'

async function main() {
  await createConnection();

  const em = getManager();

  const one = await em.findOneById(SpaceEvent, 1);
  
  console.log('ok', one);
}

main();