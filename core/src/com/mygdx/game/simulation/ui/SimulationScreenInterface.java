package com.mygdx.game.simulation.ui;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.Input;
import com.badlogic.gdx.InputMultiplexer;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.scenes.scene2d.*;
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
    private GraphicalOptionsWindow optionsWindow;
    private ControlsWindow controlsWindow;

    private Group listGroup;
    private Group infoGroup;
    private Group simSpeedGroup;
    private Group creationGroup;

    private Group masterGroup;

    private FileChooser fileChooser;

    public SimulationScreenInterface(SimulationScreen simulationScreen, InputMultiplexer multiplexer) {
        this.simulationScreen = simulationScreen;

        uiStage = new Stage(new ScreenViewport());
        masterGroup = new Group();

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

        this.simInfoWindow = new SimInfoWindow(simulationScreen,simulationScreen.getRenderer().getCamera());

        menuBarTable = new VisTable();
        menuBar = new MenuBar();

        uiStage.addActor(menuBarTable);

        menuBarTable.setFillParent(true);
        menuBarTable.left().top();
        menuBarTable.add(menuBar.getTable()).fillX().expandX().row();



        listGroup = new Group();
        listGroup.setVisible(false);
        infoGroup = new Group();
        infoGroup.setVisible(false);
        simSpeedGroup = new Group();
        simSpeedGroup.setVisible(false);
        creationGroup = new Group();
        creationGroup.setVisible(false);


        multiplexer.addProcessor(uiStage);


        addWindows();
        positionWindows();

        createMenus();
    }

    private void addWindows() {
        statsWindow = new StatsWindow(simulationScreen);
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

        this.optionsWindow = new GraphicalOptionsWindow(simulationScreen,this);
        controlsWindow = new ControlsWindow();

        toolbar = new ToolbarWindow(listGroup, infoGroup, simSpeedGroup,creationGroup);

        masterGroup.addActor(listGroup);
        masterGroup.addActor(infoGroup);
        masterGroup.addActor(simSpeedGroup);
        masterGroup.addActor(creationGroup);
        masterGroup.addActor(toolbar);

        uiStage.addActor(masterGroup);
    }

    private void positionWindows() {
        objectListWindow.setPosition(0, 100);
        statsWindow.setPosition(0,375);
        placementWindow.setPosition(Gdx.graphics.getWidth(),150);
        simInfoWindow.setPosition(Gdx.graphics.getWidth(),0);
        simSpeedWindow.setPosition(Gdx.graphics.getWidth()-700,0);
        launchToolWindow.centerWindow();
        optionsWindow.centerWindow();
        controlsWindow.centerWindow();
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

        positionWindows();
    }

    private void createMenus(){
        Menu fileMenu = new Menu("File");

        createFileMenuItems(fileMenu);

        Menu optionsMenu = new Menu("Options");

        createOptionsMenuItems(optionsMenu);

        Menu helpMenu = new Menu("Help");

        createHelpMenu(helpMenu);

        menuBar.addMenu(fileMenu);
        menuBar.addMenu(optionsMenu);
        menuBar.addMenu(helpMenu);
    }

    private void createHelpMenu(Menu helpMenu){
        final MenuItem controls = new MenuItem("Controls");

        controls.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                uiStage.addActor(controlsWindow.fadeIn());
            }
        });

        helpMenu.addItem(controls);
    }

    private void createOptionsMenuItems(Menu optionsMenu){
        MenuItem graphicsOptions = new MenuItem("Visual Options");

        graphicsOptions.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                uiStage.addActor(optionsWindow.fadeIn());
            }
        });

        optionsMenu.addItem(graphicsOptions);
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

    public void setUIVisible(boolean visible){
        masterGroup.setVisible(visible);
        menuBarTable.setVisible(visible);
    }

    public void toggleVisible(){
        setUIVisible(!masterGroup.isVisible());
    }

    public Stage getUiStage() {
        return uiStage;
    }
}
