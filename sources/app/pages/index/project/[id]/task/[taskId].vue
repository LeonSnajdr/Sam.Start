<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("title.edit", { type: $t("task.singular") }) }}</VAppBarTitle>
        <BaseActionBack />
        <TaskActionDelete v-if="task" :task class="mr-2" />
        <TaskActionSave v-if="task" @saved="taskSaved()" :disabled="!isTaskValid" :task />
    </VAppBar>
    <VContainer>
        <VCard :loading="isTaskLoading">
            <VCardText v-if="task">
                <TaskFieldContainer v-model="task" v-model:isValid="isTaskValid" />
            </VCardText>
        </VCard>
    </VContainer>
</template>

<script setup lang="ts">
const route = useRoute("index-project-id-task-taskId");

const { isTaskLoading, task, loadTask } = useTask();

const isTaskValid = ref<boolean | null>(false);

onBeforeMount(() => {
    loadTask(route.params.taskId);
});

const taskSaved = () => {
    navigateTo({ name: "index-project-id-task", params: { id: route.params.id } });
};
</script>
