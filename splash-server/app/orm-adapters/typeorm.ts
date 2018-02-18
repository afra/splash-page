import {
  ORMAdapter,
  lookup,
  HasManyRelationshipDescriptor,
  HasOneRelationshipDescriptor,
} from 'denali';
import {
  ObjectLiteral,
  getManager,
} from 'typeorm';
import { ModelRegistry, Model } from "denali-typeorm";

// type Model = Entity;
// const foo : Entity | null = null;
// type Model = {};

// declare module "denali-typeorm" {

// }

export default class TypeormAdapter extends ORMAdapter {

  get manager() {
    return getManager();
  }

  private lookupModelClass<K extends keyof ModelRegistry>(type : K) : { new(...args : any[]) : ModelRegistry[K] } {
    return lookup(`model:${type}`);
  }

  async find<K extends keyof ModelRegistry>(type: K, id: any, options?: any) {
    return this.manager.findOneById<ModelRegistry[K]>(this.lookupModelClass(type), id, options);
  }

  async queryOne<K extends keyof ModelRegistry>(type: K, query: any, options?: any) {
    return this.manager.findOne<ModelRegistry[K]>(this.lookupModelClass(type), query);
  }
  async all<K extends keyof ModelRegistry>(type: K, options?: any){
    return this.manager.find<ModelRegistry[K]>(this.lookupModelClass(type), options)
  }

  async query<K extends keyof ModelRegistry>(type: K, query: Partial<ModelRegistry[K]> | ObjectLiteral | string, options: any = {}) {
    let where;
    if(typeof query === 'string') {
      if(options.where) {
        throw 'Can not merge string where clauses';
      }
      where = query;
    } else {
      if(typeof options.where === 'string') {
        throw 'Can not merge string where clauses';
      }
      where = { ...<any>query, ...(options.where || {}) };
    }
    return this.manager.find(this.lookupModelClass(type), {...options, where});
  }

  idFor<M extends ModelRegistry[keyof ModelRegistry]>(model: Model | M) : Partial<M> | M[keyof M] {
    return this.manager.getId(model);
  }

  setId<M extends ModelRegistry[keyof ModelRegistry]>(model: Model | M, value: any) {
    throw "it is not supported to change the id"
  }

  async buildRecord<K extends keyof ModelRegistry>(type: K, data: Partial<ModelRegistry[K]>, options?: any) {
    const instance = new (this.lookupModelClass(type))(options);
    Object.assign(instance, data);
    return instance;
  }

  getAttribute<M extends ModelRegistry[keyof ModelRegistry], A extends keyof M>(model: Model | M, attribute: A) : M[A] {
    return (<M>model)[attribute];
  }

  setAttribute<M extends ModelRegistry[keyof ModelRegistry], A extends keyof M, V extends M[A]>(model: Model | M, attribute: A, value: V): boolean {
    (<M>model)[attribute] = value;
    return true;
  }

  deleteAttribute(model: Model, attribute: string): boolean {
    throw "Deleting attributes is not supported";
  }

  async getRelated<M extends ModelRegistry[keyof ModelRegistry], A extends keyof M>(model: Model | M, relationship: A, descriptor: HasManyRelationshipDescriptor | HasOneRelationshipDescriptor, options?: any): Promise<M[A]> {
    return (<M>model)[relationship];
  }
  async setRelated(model: Model, relationship: string, descriptor: HasManyRelationshipDescriptor | HasOneRelationshipDescriptor, related: any, options?: any): Promise<void> {
    throw new Error("Method not implemented.");
  }
  async addRelated(model: Model, relationship: string, descriptor: HasManyRelationshipDescriptor | HasOneRelationshipDescriptor, related: Model | Model[], options?: any): Promise<void> {
    throw new Error("Method not implemented.");
  }
  async removeRelated(model: Model, relationship: string, descriptor: HasManyRelationshipDescriptor | HasOneRelationshipDescriptor, related: Model | Model[], options?: any): Promise<void> {
    throw new Error("Method not implemented.");
  }
  async saveRecord<M extends ModelRegistry[keyof ModelRegistry]>(model: Model | M, options?: any): Promise<void> {
    await this.manager.save(model, options);
  }
  async deleteRecord<M extends ModelRegistry[keyof ModelRegistry]>(model: Model | M, options?: any): Promise<void> {
    await this.manager.remove(model);
  }
}
