import { Selector } from 'testcafe';
import config from './config.json';

const url = `${config.protocol}://${config.host}:${config.port}`;

fixture `General Flow`
    .page(url);

test('Index -> Create -> Edit -> Delete', async t => {
    const openButton = await Selector('.actions--action')
        .withText('Open')

    await t.click(openButton);
    
    const newButton = await Selector('.actions--action')
        .withText('Add');

    await t.click(newButton);

    {
        const pageTitle = await Selector('.new-todo--title').child('h1');
        const titleInput = await Selector('.new-todo--form__title');
        const contentInput = await Selector('.new-todo--form__content');
        const submitButton = await Selector('.new-todo--actions__submit');

        await t.expect(pageTitle.innerText).contains('New');

        await t.typeText(titleInput, 'test title');
        await t.typeText(contentInput, 'test content ');
        await t.click(submitButton);
    }

    const newItem = await Selector('.todo-container').child('.todo-item').nth(-1);

    const newItemHeader = await newItem.child('.todo-item--header');
    const newItemContent = await newItem.child('.todo-item--content');
    const editButton = await newItemHeader.child('.todo-item--actions').child('.todo-item--actions__edit');
    const deleteButton = await newItemHeader.child('.todo-item--actions').child('.todo-item--actions__delete');

    await t.expect(newItemHeader.innerText).contains('test title');
    await t.expect(newItemContent.innerText).contains('test content');

    await t.hover(newItem);

    await t.click(editButton);

    {
        const pageTitle = await Selector('.edit-todo--title').child('h1');
        const titleInput = await Selector('.edit-todo--form__title');
        const contentInput = await Selector('.edit-todo--form__content');
        const submitButton = await Selector('.edit-todo--actions__submit');
    
        await t.expect(pageTitle.innerText).contains('Edit');

        await t.typeText(titleInput, 'editted test title', { replace: true });
        await t.typeText(contentInput, 'editted test content', { replace: true });
        await t.click(submitButton);
    }

    await t.expect(newItemHeader.innerText).contains('editted test title');
    await t.expect(newItemContent.innerText).contains('editted test content');

    await t.hover(newItem);
    await t.click(deleteButton);
})
