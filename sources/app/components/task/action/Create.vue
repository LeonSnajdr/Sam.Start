<template>
    <BaseBtnIcon @click="createTask()" :loading="isLoading" color="success" icon="mdi-plus" variant="flat">
        {{ $t("action.create") }}
    </BaseBtnIcon>
</template>

<script setup lang="ts">
const emit = defineEmits<{
    created: [task: TaskContract];
}>();

const props = defineProps<{
    task: TaskCreateContract;
}>();

const notify = useNotify();
const { t } = useI18n();

const taskStore = useTaskStore();

const isLoading = ref(false);

const createTask = async () => {
    isLoading.value = true;

    const createResult = await commands.taskCreate(props.task);

    isLoading.value = false;

    if (createResult.status == "error") {
        notify.error(t("action.create.error", { type: t("task.singular"), name: props.task.name }), { error: createResult.error });
        return;
    }

    notify.success(t("action.create.success", { type: t("task.singular"), name: props.task.name }));

    taskStore.loadAll();

    emit("created", createResult.data);
};
</script>
