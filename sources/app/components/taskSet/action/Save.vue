<template>
    <BaseBtnIcon @click="saveTaskSet()" :loading="isLoading" color="success" icon="mdi-content-save" variant="flat">
        {{ $t("action.save") }}
    </BaseBtnIcon>
</template>

<script setup lang="ts">
const props = defineProps<{
    taskSet: TaskSetContract;
}>();

const notify = useNotify();
const { t } = useI18n();

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const isLoading = ref(false);

const saveTaskSet = async () => {
    isLoading.value = true;

    const updateContract: TaskSetUpdateContract = {
        id: props.taskSet.id,
        name: props.taskSet.name,
        tasks: props.taskSet.tasks
    };

    const saveResult = await commands.taskSetUpdateOne(updateContract);

    isLoading.value = false;

    if (saveResult.status == "error") {
        notify.error(t("action.save.error", { type: t("taskSet.singular"), name: props.taskSet.name }), { error: saveResult.error });
        return;
    }

    notify.success(t("action.save.success", { type: t("taskSet.singular"), name: props.taskSet.name }));

    navigateTo({ name: "index-project-id-taskSet", params: { id: selectedProject.value.id } });
};
</script>
