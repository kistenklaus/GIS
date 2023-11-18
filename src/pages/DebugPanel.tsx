import RandomNumber from "../components/RandomNumber";
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import ChevronRightIcon from '@mui/icons-material/ChevronRight';
import { TreeView } from '@mui/x-tree-view/TreeView';
import { TreeItem } from '@mui/x-tree-view/TreeItem';
import Graph from "../components/Graph";
import Stack from "@mui/material/Stack/Stack";

function DebugPanel() {
    return (<><FirstComponent/> 
    <Stack spacing={2}>
      <Graph/>
      <Graph/>
      <Graph/>
      <Graph/>
    </Stack>
    </>);
};

export default DebugPanel;


function FirstComponent() {
    return (
        <TreeView
            aria-label="file system navigator"
            defaultCollapseIcon={<ExpandMoreIcon />}
            defaultExpandIcon={<ChevronRightIcon />}
            sx={{ height: 240, flexGrow: 1, maxWidth: 400, overflowY: 'auto' }}
        >
            <TreeItem nodeId="1" label="Applications">
                <TreeItem nodeId="2" label="Calendar" />
            </TreeItem>
            <TreeItem nodeId="5" label="Documents">
                <TreeItem nodeId="10" label="OSS" />
                <TreeItem nodeId="6" label="MUI">
                    <TreeItem nodeId="8" label="index.js" />
                </TreeItem>
            </TreeItem>
        </TreeView>
    );
}
