import * as React from 'react';
import ListItemIcon from '@mui/material/ListItemIcon';
import ListItemText from '@mui/material/ListItemText';
import TerminalIcon from '@mui/icons-material/Terminal';
import GamesIcon from '@mui/icons-material/Games';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import ChevronRightIcon from '@mui/icons-material/ChevronRight';
import clsx from 'clsx';
import {Link as RouterLink, LinkProps as RouterLinkProps, useNavigate,} from 'react-router-dom';
import {Box, ListItemButton, Typography} from '@mui/material';
import {TreeItem, TreeItemContentProps, TreeItemProps, TreeView, useTreeItem} from '@mui/x-tree-view';

interface ListItemLinkProps {
    icon?: React.ReactElement;
    primary: string;
    to: string;
}

const CustomContent = React.forwardRef(function CustomContent(
    props: TreeItemContentProps,
    ref,
) {
    const {
        classes,
        className,
        label,
        nodeId,
        icon: iconProp,
        expansionIcon,
        displayIcon,
    } = props;

    const {
        disabled,
        expanded,
        selected,
        focused,
        handleExpansion,
        preventSelection,
    } = useTreeItem(nodeId);

    const icon = iconProp || expansionIcon || displayIcon;
    const navigate = useNavigate();

    const handleMouseDown = (event: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
        preventSelection(event);
    };

    const handleExpansionClick = (
        event: React.MouseEvent<HTMLDivElement, MouseEvent>,
    ) => {
        handleExpansion(event);
    };

    const handleSelectionClick = () => {
        navigate(nodeId)
    };

    return (
        <div
            className={clsx(className, classes.root, {
                [classes.expanded]: expanded,
                [classes.selected]: selected,
                [classes.focused]: focused,
                [classes.disabled]: disabled,
            })}
            onMouseDown={handleMouseDown}
            ref={ref as React.Ref<HTMLDivElement>}
        >
            <div onClick={handleExpansionClick} className={classes.iconContainer}>
                {icon}
            </div>
            <Typography
                onClick={handleSelectionClick}
                component="div"
                className={classes.label}
            >
                {label}
            </Typography>
        </div>

    );
});
//
const CustomTreeItem = React.forwardRef(function CustomTreeItem(
    props: TreeItemProps,
    ref: React.Ref<HTMLLIElement>,
) {
    return <TreeItem ContentComponent={CustomContent} {...props} ref={ref}/>;
});


const Link = React.forwardRef<HTMLAnchorElement, RouterLinkProps>(function Link(
    itemProps,
    ref,
) {
    return <RouterLink ref={ref} {...itemProps} role={undefined}/>;
});

function ListItemButtonLink(props: ListItemLinkProps) {
    const {icon, primary, to} = props;

    return (
        <li>
            <ListItemButton component={Link} to={to}>
                {icon ? <ListItemIcon>{icon}</ListItemIcon> : null}
                <ListItemText primary={primary}/>
            </ListItemButton>
        </li>
    );
}

export const RouterList = (
    <React.Fragment>
        <ListItemButtonLink to="/" primary="Control Panel" icon={<GamesIcon/>}/>
        <ListItemButtonLink to="/DebugPanel" primary="Debug Panel" icon={<TerminalIcon/>}/>
    </React.Fragment>
);

export function NodeList(nodes: string[]) {
    return (
        <React.Fragment>
            <Box sx={{minHeight: 180, flexGrow: 1, maxWidth: 300}}>
                <TreeView
                    aria-label="icon expansion"
                    defaultCollapseIcon={<ExpandMoreIcon/>}
                    defaultExpandIcon={<ChevronRightIcon/>}
                >
                    {nodes.map((node) =>
                        <CustomTreeItem nodeId={node} label={node}>
                            {/*Label has to euqal the page name!*/}
                            <CustomTreeItem nodeId={node + "/Object"} label="Object"/>
                            <CustomTreeItem nodeId={node + "/Command"} label="Command"/>
                        </CustomTreeItem>)}

                </TreeView>
            </Box>
        </React.Fragment>
    );
}
