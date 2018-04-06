package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.utils.BufferUtils;
import com.badlogic.gdx.utils.Disposable;

import java.nio.IntBuffer;

public class Query implements Disposable{
    private final int id;
    private final int type;

    private boolean inUse = false;

    public Query(int type){
        this.type = type;
        int[] arr = new int[1];
        Gdx.gl30.glGenQueries(1,arr,0);
        id = arr[0];
    }

    public void start(){
        Gdx.gl30.glBeginQuery(type, id);
        inUse = true;
    }

    public void end(){
        Gdx.gl30.glEndQuery(type);
    }

    public boolean isResultReady(){
        IntBuffer result = BufferUtils.newIntBuffer(1);
        Gdx.gl30.glGetQueryObjectuiv(id,Gdx.gl30.GL_QUERY_RESULT_AVAILABLE, result);
        return result.get() == Gdx.gl.GL_TRUE;
    }

    public int getResult(){
        inUse = false;
        IntBuffer result = BufferUtils.newIntBuffer(1);
        Gdx.gl30.glGetQueryObjectuiv(id, Gdx.gl30.GL_QUERY_RESULT, result);
        return result.get();
    }

    public boolean isInUse(){
        return inUse;
    }

    @Override
    public void dispose() {
        Gdx.gl30.glDeleteQueries(1, new int[]{id},0);
    }
}
