import "reflect-metadata";
import {
  Entity,
  Column,
  PrimaryGeneratedColumn,
  CreateDateColumn
} from 'typeorm';

{
  const fileSymbol = Symbol.for('/full/path/to/file');
  if(global[fileSymbol]) {
    throw "double loading file";
  }
  global[fileSymbol] = true;
}

@Entity()
export class SpaceEvent {
  @PrimaryGeneratedColumn()
  id: number;

  @Column('boolean')
  open: boolean;

  @CreateDateColumn()
  createdAt: Date;
}

global.SpaceEvent = SpaceEvent;

declare module "denali-typeorm" {
  interface Model {}
  interface ModelRegistry {
    'space-event': SpaceEvent;
  }
}
