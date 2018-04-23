package com.mygdx.game.simulation.ui;

import com.badlogic.gdx.InputMultiplexer;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.scenes.scene2d.*;
import com.badlogic.gdx.scenes.scene2d.ui.TextField;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.badlogic.gdx.utils.viewport.ScreenViewport;
import com.kotcrab.vis.ui.widget.*;
import com.kotcrab.vis.ui.widget.file.FileChooser;
import com.kotcrab.vis.ui.widget.file.FileTypeFilter;
import com.kotcrab.vis.ui.widget.file.SingleFileChooserListener;
import com.mygdx.game.simulation.SaveFileManager;
import com.mygdx.game.simulation.SimulationObject;
import com.mygdx.game.simulation.SimulationScreen;
import com.mygdx.game.simulation.logic.algorithms.NBodyAlgorithm;
import com.mygdx.game.simulation.ui.windows.*;
import javafx.scene.control.ToolBar;

import java.io.File;

/**
 * Created by fraayala19 on 4/18/18.
 */
public class SimulationScreenInterface {
    private SimulationScreen simulationScreen;
    private Stage uiStage;
    private MenuBar menuBar;
    private VisTable menuBarTable;

    private ObjectListWindow objectListWindow;
    private StatsWindow statsWindow;
    private ToolbarWindow toolbar;
    private SimInfoWindow simInfoWindow;
    private SimSpeedWindow simSpeedWindow;
    private PlacementWindow placementWindow;
    private LaunchToolWindow launchToolWindow;

    private Group listGroup;
    private Group infoGroup;
    private Group simSpeedGroup;
    private Group creationGroup;

    private FileChooser fileChooser;

    public SimulationScreenInterface(SimulationScreen simulationScreen, InputMultiplexer multiplexer, NBodyAlgorithm nBodyAlgorithm) {
        this.simulationScreen = simulationScreen;

        uiStage = new Stage(new ScreenViewport());

        FileChooser.setDefaultPrefsName("com.mygdx.game.simulation.filechooser");
        fileChooser = new FileChooser(FileChooser.Mode.OPEN);
        setupFileChooser();

        uiStage.getRoot().addCaptureListener(new InputListener() {
            public boolean touchDown(InputEvent event, float x, float y, int pointer, int button) {

                if (uiStage.hit(x,y,true) == null) {
                    uiStage.setKeyboardFocus(null);
                    uiStage.setScrollFocus(null);
                    return false;
                }
                return true;
            }
        });

        this.simInfoWindow = new SimInfoWindow(nBodyAlgorithm);

        menuBarTable = new VisTable();
        menuBar = new MenuBar();

        uiStage.addActor(menuBarTable);

        menuBarTable.setFillParent(true);
        menuBarTable.left().top();
        menuBarTable.add(menuBar.getTable()).fillX().expandX().row();

        listGroup = new Group();
        infoGroup = new Group();
        simSpeedGroup = new Group();
        creationGroup = new Group();

        multiplexer.addProcessor(uiStage);


        addWindows();
        positionWindows();

        createMenus();
    }

    private void addWindows() {
        statsWindow = new StatsWindow();
        listGroup.addActor(statsWindow);
        objectListWindow = new ObjectListWindow(simulationScreen, statsWindow);
        listGroup.addActor(objectListWindow);

        infoGroup.addActor(simInfoWindow);

        simSpeedWindow = new SimSpeedWindow();
        simSpeedGroup.addActor(simSpeedWindow);

        launchToolWindow = new LaunchToolWindow(simulationScreen.getRenderer().getPlacementManager());
        placementWindow = new PlacementWindow(launchToolWindow,simulationScreen.getRenderer(),
                simulationScreen.getRenderer().getPlacementManager());
        creationGroup.addActor(launchToolWindow);
        creationGroup.addActor(placementWindow);

        toolbar = new ToolbarWindow(listGroup, infoGroup, simSpeedGroup,creationGroup);

        uiStage.addActor(listGroup);
        uiStage.addActor(infoGroup);
        uiStage.addActor(simSpeedGroup);
        uiStage.addActor(creationGroup);
        uiStage.addActor(toolbar);
    }

    private void positionWindows() {
        objectListWindow.setPosition(0, 0);
    }

    public void addObject(SimulationObject simulationObject) {
        objectListWindow.addObject(simulationObject);
    }

    public void removeObject(SimulationObject simulationObject) {
        objectListWindow.removeobject(simulationObject);
    }

    public void update() {
        uiStage.act();
        statsWindow.update();
    }

    public void render() {
        uiStage.draw();
    }

    public void resize(int width, int height) {
        uiStage.getViewport().update(width, height, true);

        menuBarTable.clearChildren();
        menuBarTable.add(menuBar.getTable()).fillX().expandX().row();
    }

    private void createMenus(){
        Menu fileMenu = new Menu("File");

        createFileMenuItems(fileMenu);

        menuBar.addMenu(fileMenu);
    }

    private void setupFileChooser(){
        fileChooser.setSelectionMode(FileChooser.SelectionMode.FILES);
        final FileTypeFilter fileTypeFilter = new FileTypeFilter(true);
        fileTypeFilter.addRule("Text files (*.txt)","txt");

        fileChooser.setFileTypeFilter(fileTypeFilter);

        fileChooser.setListener(new SingleFileChooserListener() {
            @Override
            protected void selected(FileHandle file) {
                if(fileChooser.getMode()== FileChooser.Mode.OPEN){
                    SaveFileManager.loadGame(simulationScreen, simulationScreen.getRenderer(),file);
                } else if(fileChooser.getMode()== FileChooser.Mode.SAVE){
                    SaveFileManager.saveGame(simulationScreen, file);
                }
            }
        });
    }

    private void createFileMenuItems(Menu fileMenu){
        MenuItem load = new MenuItem("Load");

        load.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                fileChooser.setMode(FileChooser.Mode.OPEN);
                uiStage.addActor(fileChooser.fadeIn());
            }
        });

        MenuItem save = new MenuItem("Save");

        save.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                fileChooser.setMode(FileChooser.Mode.SAVE);
                uiStage.addActor(fileChooser.fadeIn());
            }
        });

        fileMenu.addItem(load);
        fileMenu.addItem(save);
    }

    public Stage getUiStage() {
        return uiStage;
    }
}
