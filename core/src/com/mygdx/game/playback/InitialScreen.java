package com.mygdx.game.playback;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 5/1/18   ==
* =====================================================================
* ==      Project: orbit-simulator-2    ==
* =====================================================================

*/

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.GL20;
import com.badlogic.gdx.graphics.OrthographicCamera;
import com.badlogic.gdx.scenes.scene2d.Stage;
import com.badlogic.gdx.scenes.scene2d.ui.Table;
import com.badlogic.gdx.scenes.scene2d.ui.TextField;
import com.badlogic.gdx.utils.Array;
import com.badlogic.gdx.utils.viewport.ScreenViewport;
import com.kotcrab.vis.ui.widget.VisSlider;
import com.kotcrab.vis.ui.widget.VisTextButton;
import com.kotcrab.vis.ui.widget.file.FileChooser;
import com.kotcrab.vis.ui.widget.file.FileChooserAdapter;
import com.kotcrab.vis.ui.widget.file.FileTypeFilter;
import com.mygdx.game.BaseScreen;
import com.mygdx.game.Boot;
import com.mygdx.game.playback.ui.CustomFileChooser;

import java.util.TimerTask;

public class InitialScreen extends BaseScreen {

    private CustomFileChooser fileChooser;
    private Stage stage;



    public InitialScreen(final Boot boot){
        super(boot);

        fileChooser = new CustomFileChooser(FileChooser.Mode.OPEN);
        fileChooser.setSelectionMode(FileChooser.SelectionMode.FILES);

        final FileTypeFilter fileTypeFilter = new FileTypeFilter(true);
        fileTypeFilter.addRule("NBD files (*.txt)","nbd");

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


        stage = new Stage(new ScreenViewport());
        Table table = new Table();
        table.setWidth(stage.getWidth());
        table.setHeight(stage.getHeight());
        table.setFillParent(true);

        table.addActor(fileChooser);
        stage.addActor(table);

        Gdx.input.setInputProcessor(stage);

    }
    @Override
    public void show() {

    }

    @Override
    public void render(float delta) {
        Gdx.gl.glClearColor(0, 0, 0, 1);
        Gdx.gl.glClear(GL20.GL_COLOR_BUFFER_BIT);

        stage.act(Gdx.graphics.getDeltaTime());
        stage.draw();
    }

    @Override
    public void resize(int width, int height) {

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
        fileChooser = null;
        stage.dispose();
    }
}
