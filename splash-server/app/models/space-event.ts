import { Entity, Column, PrimaryGeneratedColumn, ManyToOne, getConnection } from 'typeorm';
import { /*  attr, hasOne, hasMany */ } from 'denali';
import ApplicationModel from './application';

export class SpaceEvent {
  @PrimaryGeneratedColumn()
  id: number;
}

// export default infer(SpaceEvent)

// async function infer(cls : any) {
//   const connenv = {
//     "type": "postgres",
//     "host": "localhost",
//     "port": 5432,
//     "username": "postgres",
//     "password": "postgres",
//     "database": "splash-page",
//     "entities": ["dist/app/models/*{.ts,.js}"]
//   };

//   debugger;

//   const conn = createConnection(connenv);
//   console.log('hi');
// }

// export default class SpaceEvent extends ApplicationModel {

//   static schema = {};

// }
