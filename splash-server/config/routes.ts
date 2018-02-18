import {
  Router
} from 'denali';

export default function drawRoutes(router : Router) {

  router.get('/', 'index');
  
  router.namespace('/api/v1/complex/', router => {
    router.get('space-events', 'space-events/list');
    router.get('space-events/:id', 'space-events/show');
  });

}
