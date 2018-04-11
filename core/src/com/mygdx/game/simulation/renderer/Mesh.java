package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.utils.BufferUtils;
import com.badlogic.gdx.utils.Disposable;

import java.nio.Buffer;
import java.nio.FloatBuffer;
import java.nio.IntBuffer;
import java.util.ArrayList;
import java.util.List;

import static com.badlogic.gdx.graphics.GL20.GL_ARRAY_BUFFER;

/**
 * Created by fraayala19 on 3/15/18.
 */
public class Mesh implements Disposable{
    protected int[] indices;
    protected float[] vertices;
    protected float[] texCoords;
    protected float[] normals;
    protected int diffuseTexture;

    private int VAO;
    private int EBO;

    private List<Integer> vboList;

    private boolean useTexture = true;

    public Mesh(int[] indices, float[] vertices, float[] texCoords, float[] normals, int diffuseTexture) {
        this.indices = indices;
        this.vertices = vertices;
        this.texCoords = texCoords;
        this.normals = normals;
        this.diffuseTexture = diffuseTexture;


        vboList = new ArrayList<Integer>();

        setupMesh();
    }

    protected Mesh copy(){
        return new Mesh(indices,vertices,texCoords, normals,diffuseTexture);
    }

    protected void setupMesh(){
        int[] tempv = new int[1];
        Gdx.gl30.glGenVertexArrays(1, tempv, 0);
        VAO = tempv[0];


        Gdx.gl30.glBindVertexArray(VAO);

        int VBO = Gdx.gl.glGenBuffer();
        vboList.add(VBO);
        Gdx.gl.glBindBuffer(GL_ARRAY_BUFFER, VBO);
        Gdx.gl.glBufferData(GL_ARRAY_BUFFER, vertices.length*4, toFloatBuffer(vertices), Gdx.gl.GL_STATIC_DRAW);
        Gdx.gl.glEnableVertexAttribArray(0);
        Gdx.gl.glVertexAttribPointer(0, 3, Gdx.gl.GL_FLOAT, false, 0, 0);

        VBO = Gdx.gl.glGenBuffer();
        vboList.add(VBO);
        Gdx.gl.glBindBuffer(GL_ARRAY_BUFFER, VBO);
        Gdx.gl.glBufferData(GL_ARRAY_BUFFER, texCoords.length*4, toFloatBuffer(texCoords), Gdx.gl.GL_STATIC_DRAW);
        Gdx.gl.glEnableVertexAttribArray(1);
        Gdx.gl.glVertexAttribPointer(1, 2, Gdx.gl.GL_FLOAT, false, 0, 0);

        VBO = Gdx.gl.glGenBuffer();
        vboList.add(VBO);
        Gdx.gl.glBindBuffer(GL_ARRAY_BUFFER, VBO);
        Gdx.gl.glBufferData(GL_ARRAY_BUFFER, normals.length*4, toFloatBuffer(normals), Gdx.gl.GL_STATIC_DRAW);
        Gdx.gl.glEnableVertexAttribArray(2);
        Gdx.gl.glVertexAttribPointer(2, 3, Gdx.gl.GL_FLOAT, false, 0, 0);

        EBO = Gdx.gl.glGenBuffer();
        Gdx.gl.glBindBuffer(Gdx.gl.GL_ELEMENT_ARRAY_BUFFER, EBO);
        Gdx.gl.glBufferData(Gdx.gl.GL_ELEMENT_ARRAY_BUFFER, indices.length*4,toIntBuffer(indices), Gdx.gl.GL_STATIC_DRAW);

        Gdx.gl.glDisableVertexAttribArray(0);
        Gdx.gl.glDisableVertexAttribArray(1);
        Gdx.gl.glDisableVertexAttribArray(2);

        Gdx.gl.glBindBuffer(GL_ARRAY_BUFFER, 0);
        Gdx.gl30.glBindVertexArray(0);
    }

    public void render(Shader shader){

        shader.use();

        if(useTexture) {
            Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE0);
            shader.setInt("diffuseTex", 0);
            Gdx.gl.glBindTexture(Gdx.gl.GL_TEXTURE_2D, diffuseTexture);
            Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE0);
        }

        Gdx.gl30.glBindVertexArray(VAO);

        Gdx.gl.glEnableVertexAttribArray(0);
        Gdx.gl.glEnableVertexAttribArray(1);
        Gdx.gl.glEnableVertexAttribArray(2);


        Gdx.gl.glBindBuffer(Gdx.gl.GL_ELEMENT_ARRAY_BUFFER, EBO);
        Gdx.gl.glDrawElements(Gdx.gl.GL_TRIANGLES, indices.length, Gdx.gl.GL_UNSIGNED_INT, 0);

        Gdx.gl.glDisableVertexAttribArray(0);
        Gdx.gl.glDisableVertexAttribArray(1);
        Gdx.gl.glDisableVertexAttribArray(2);

        Gdx.gl30.glBindVertexArray(0);
    }

    @Override
    public void dispose() {
        for(Integer i : this.vboList){
            Gdx.gl.glDeleteBuffer(i);
        }
        Gdx.gl.glDeleteBuffer(EBO);
        Gdx.gl30.glDeleteVertexArrays(1, new int[VAO], 0);
    }

    protected IntBuffer toIntBuffer(int[] arr){
        IntBuffer buff = BufferUtils.newIntBuffer(arr.length);
        buff.put(arr);
        buff.flip();
        return buff;
    }

    protected FloatBuffer toFloatBuffer(float[] arr){
        FloatBuffer buff = BufferUtils.newFloatBuffer(arr.length);
        buff.put(arr);
        buff.flip();
        return buff;
    }

    public void setDiffuseTexture(int diffuseTexture) {
        this.diffuseTexture = diffuseTexture;
    }

    public void setUseTexture(boolean useTexture) {
        this.useTexture = useTexture;
    }
}
