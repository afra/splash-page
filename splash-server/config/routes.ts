import {
  Router
} from 'denali';

export default function drawRoutes(router : Router) {

  router.get('/', 'index');
  
  router.namespace('/api/v1/complex/', router => {
    router.get('space-events', 'space-events/list');
    router.post('space-events/', 'space-events/create');
    router.get('space-events/:id', 'space-events/show');
    router.patch('space-events/:id', 'space-events/update');
  });

}
