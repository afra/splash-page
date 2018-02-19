import {
  createConnection,
  getConnectionOptions,
} from 'typeorm';
import {
  container,
} from "denali";

export default {
  name: 'typeorm-connection',

  async initialize(app): Promise<void> {
    container.setOption('entity', 'singleton', false);

    // override the entities with classes loaded by denali
    // https://github.com/denali-js/denali/issues/430
    const options = await getConnectionOptions();
    const entityLookups = container.lookupAll('entity');
    console.log(entityLookups);

    const entities : ({new(): any})[] = [];

    Object.keys(entityLookups)
      .map(k => entityLookups[k])
      .forEach(module => {
        if(typeof module === 'function') {
          entities.push(module);
        } else {
          Object.keys(module)
            .map(k => module[k])
            .forEach(exp => {
              entities.push(exp);
            });
        }
      });

    Object.assign(options, { entities });
    
    await createConnection(options);
    console.log('Established database connection');
  }
}