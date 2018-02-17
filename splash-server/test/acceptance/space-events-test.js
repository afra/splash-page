import { setupAcceptanceTest } from 'denali';

const test = setupAcceptanceTest();

test('POST /space-events > creates a space event', async (t) => {
  let result = await t.context.app.post('/space-events', {
    // Add the space event payload here
  });

  t.is(result.status, 201);
  // t.is(result.body.foo, 'bar');
});

test('GET /space-events > should list space events', async (t) => {
  let result = await t.context.app.get('/space-events');

  t.is(result.status, 200);
  // t.is(result.body.foo, 'bar');
});

test('GET /space-events/:id > should show a space event', async (t) => {
  let { body } = await t.context.app.post('/space-events', {
    // Add the space event payload here
  });
  let id = body.data.id;

  let result = await t.context.app.get(`/space-events/${ id }`);

  t.is(result.status, 200);
  // t.is(result.body.foo, 'bar');
});

test('PATCH /space-events/:id > should update a space event', async (t) => {
  let { body } = await t.context.app.post('/space-events', {
    // Add the space event payload here
  });
  let id = body.data.id;

  let result = await t.context.app.patch(`/space-events/${ id }`, {
    // Add the space event payload here
  });

  t.is(result.status, 200);
  // t.is(result.body.foo, 'bar');
});

test('DELETE /space-events/:id > should delete a space event', async (t) => {
  let { body } = await t.context.app.post('/space-events', {
    // Add the space event payload here
  });
  let id = body.data.id;

  let result = await t.context.app.delete(`/space-events/${ id }`);

  t.is(result.status, 204);
});
