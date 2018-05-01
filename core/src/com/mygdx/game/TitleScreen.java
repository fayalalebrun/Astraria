package com.mygdx.game;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.Stage;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.badlogic.gdx.scenes.scene2d.utils.TextureRegionDrawable;
import com.badlogic.gdx.utils.Array;
import com.badlogic.gdx.utils.viewport.ScreenViewport;
import com.kotcrab.vis.ui.util.TableUtils;
import com.kotcrab.vis.ui.widget.VisImage;
import com.kotcrab.vis.ui.widget.VisTable;
import com.kotcrab.vis.ui.widget.VisTextButton;
import com.kotcrab.vis.ui.widget.file.FileChooser;
import com.kotcrab.vis.ui.widget.file.FileChooserAdapter;
import com.kotcrab.vis.ui.widget.file.FileTypeFilter;
import com.mygdx.game.playback.PlayBackScreen;
import com.mygdx.game.simulation.SimulationScreen;

/**
 * Created by fraayala19 on 4/30/18.
 */
public class TitleScreen extends BaseScreen{
    private Stage uiStage;

    private VisTable mainTable;

    private VisTextButton simButton;
    private VisTextButton playButton;

    private VisImage logo;
    private VisImage background;

    private FileChooser fileChooser;



    public TitleScreen(final Boot boot) {
        super(boot);
        uiStage = new Stage(new ScreenViewport());
        Gdx.input.setInputProcessor(uiStage);

        mainTable = new VisTable();

        TableUtils.setSpacingDefaults(mainTable);

        uiStage.addActor(mainTable);

        mainTable.setFillParent(true);

        background = new VisImage(convertToDrawable(Boot.manager.get("stars_milky_way.jpg",Texture.class)));
        logo = new VisImage(convertToDrawable(Boot.manager.get("logo/ASTRARIA LOGO-01.png",Texture.class)));

        simButton = new VisTextButton("Real-Time Simulation");
        playButton = new VisTextButton("Pre-Generated Simulation");

        simButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                boot.setScreen(new SimulationScreen(boot,Gdx.files.internal("examples/Solar_System_2K.txt").file().getAbsolutePath()));
            }
        });

        playButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                //boot.setScreen(new InitialScreen(boot));
                uiStage.addActor(fileChooser.fadeIn());
            }
        });

        fileChooser = new FileChooser(FileChooser.Mode.OPEN);
        //fileChooser.fadeOut(0);
        fileChooser.setSelectionMode(FileChooser.SelectionMode.FILES);

        final FileTypeFilter fileTypeFilter = new FileTypeFilter(true);
        fileTypeFilter.addRule("NBD files (*.nbd)","nbd");

        fileChooser.setFileTypeFilter(fileTypeFilter);
        fileChooser.setListener(new FileChooserAdapter() {



            @Override
            public void selected (Array<FileHandle> file) {
                fileChooser.fadeOut(0);
                if (file.get(0).file().exists()){
                    try {
                        boot.setScreen(new PlayBackScreen(boot,file.get(0).file().getCanonicalPath()));
                    }catch (Exception e){
                        e.printStackTrace();
                    }
                }
            }

            @Override
            public void canceled() {
                super.canceled();
            }
        });

        //uiStage.addActor(fileChooser);

        addWidgets();
    }

    private void addWidgets(){

        mainTable.add(logo).size(logo.getWidth()/2,logo.getHeight()/2);
        mainTable.row();
        mainTable.add(simButton);
        mainTable.row();
        mainTable.add(playButton);
        mainTable.row();
    }

    @Override
    public void show() {

    }

    @Override
    public void render(float delta) {
        uiStage.act();

        Gdx.gl.glClear(Gdx.gl.GL_COLOR_BUFFER_BIT);
        uiStage.getBatch().begin();
        background.draw(uiStage.getBatch(),1.0f);
        uiStage.getBatch().end();

        uiStage.draw();
    }

    @Override
    public void resize(int width, int height) {
        uiStage.getViewport().update(width,height,true);
        uiStage.clear();
        uiStage.addActor(mainTable);
    }

    @Override
    public void pause() {

    }

    @Override
    public void resume() {

    }

    @Override
    public void hide() {

    }

    @Override
    public void dispose() {

    }

    private TextureRegionDrawable convertToDrawable(Texture texture){
        return new TextureRegionDrawable(new TextureRegion(texture));
    }
}
