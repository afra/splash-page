import { createConnection } from 'typeorm';

export default {
  name: 'typeorm-connection',

  async initialize(): Promise<void> {
    await createConnection();
  }
}