<template>
    <BaseDrawerCreate v-model="isDrawerOpen" :emptyElement :type="$t('task.singular')">
        <template #actions="{ isFormValid, element }">
            <TaskActionCreate @created="taskCreated" :disabled="!isFormValid" :task="element" />
        </template>
        <template #fields="{ element: task }">
            <TaskFieldName v-model="task.name" autofocus />
            <TaskFieldTabName v-model="task.tabName" />
            <VRow>
                <VCol>
                    <TaskFieldNoExit v-model="task.noExit" />
                </VCol>
                <VCol>
                    <TaskFieldForceKill v-model="task.forceKill" />
                </VCol>
            </VRow>
            <TaskFieldHistoryPersistence v-model="task.historyPersistence" />
            <TaskFieldCommandTiles v-model="task.commandTiles" />
            <TaskFieldWorkingDirTiles v-model="task.workingDirTiles" />
        </template>
    </BaseDrawerCreate>
</template>

<script setup lang="ts">
const emit = defineEmits<{
    created: [id: TaskContract];
}>();

const isDrawerOpen = defineModel<boolean>({ required: true });

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const emptyElement: TaskCreateContract = {
    projectId: selectedProject.value.id,
    name: "",
    tabName: null,
    noExit: true,
    forceKill: false,
    historyPersistence: "Never",
    workingDirTiles: [] as PlaceholderInsertTileContract[],
    commandTiles: [] as PlaceholderInsertTileContract[]
};

const taskCreated = (task: TaskContract) => {
    isDrawerOpen.value = false;
    emit("created", task);
};
</script>
