<template>
    <BaseBtnIcon :loading="isLoading" icon="mdi-delete" variant="flat">
        {{ $t("action.delete") }}
        <BaseDialogConfirm
            @confirm="placeholderDelete"
            :message="$t('action.delete.description', { type: $t('placeholder.singular'), name: placeholder.name })"
            :title="$t('action.delete.title', { type: $t('placeholder.singular') })"
            icon="mdi-label"
            iconColor="error"
        />
    </BaseBtnIcon>
</template>

<script setup lang="ts">
const props = defineProps<{
    placeholder: PlaceholderContract;
}>();

const notify = useNotify();
const { t } = useI18n();

const placeholderStore = usePlaceholderStore();
const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const isDialogOpen = ref(false);
const isLoading = ref(false);

const placeholderDelete = async () => {
    isLoading.value = true;

    const deleteResult = await commands.placeholderDeleteOne(props.placeholder.id);

    isLoading.value = false;

    if (deleteResult.status == "error") {
        notify.error(t("action.delete.error", { type: t("placeholder.singular"), name: props.placeholder.name }), { error: deleteResult.error });
        return;
    }

    notify.success(t("action.delete.success", { type: t("placeholder.singular"), name: props.placeholder.name }));

    placeholderStore.loadAll();

    isDialogOpen.value = false;

    navigateTo({ name: "index-project-id-placeholder", params: { id: selectedProject.value.id } });
};
</script>
