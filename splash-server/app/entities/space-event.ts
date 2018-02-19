import "reflect-metadata";
import {
  Entity,
  Column,
  PrimaryGeneratedColumn,
  CreateDateColumn
} from 'typeorm';

@Entity()
export class SpaceEvent {
  @PrimaryGeneratedColumn()
  id: number;

  @Column('boolean')
  open: boolean;

  @CreateDateColumn()
  createdAt: Date;
}

declare module "denali-typeorm" {
  interface Model {}
  interface ModelRegistry {
    'space-event': SpaceEvent;
  }
}
