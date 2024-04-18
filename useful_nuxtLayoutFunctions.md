```ts
function enableCustomLayout () {
  setPageLayout('custom')
}
definePageMeta({
  layout: false,
});

<h1 @click="enableCustomLayout"></h1>

definePageMeta({
  layout: 'default'
})
```