package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.utils.BufferUtils;
import com.badlogic.gdx.utils.Disposable;

import java.nio.FloatBuffer;
import java.nio.IntBuffer;

/**
 * Created by fraayala19 on 3/15/18.
 */
public class Mesh implements Disposable{
    private IntBuffer indices;
    private FloatBuffer vertices;
    private FloatBuffer texCoords;
    private FloatBuffer normals;
    private int diffuseTexture;

    private int VAO;
    private int VBO;
    private int EBO;

    public Mesh(IntBuffer indices, FloatBuffer vertices, FloatBuffer texCoords, FloatBuffer normals, int diffuseTexture) {
        this.indices = indices;
        this.vertices = vertices;
        this.texCoords = texCoords;
        this.normals = normals;
        this.diffuseTexture = diffuseTexture;

        setupMesh();
    }

    private void setupMesh(){
        int[] tempv = new int[1];
        Gdx.gl30.glGenVertexArrays(1, tempv, 0);
        VAO = tempv[0];

        VBO = Gdx.gl.glGenBuffer();

        EBO = Gdx.gl.glGenBuffer();

        int numOfFloats = vertices.remaining()+texCoords.remaining()+normals.remaining();

        FloatBuffer buffer = BufferUtils.newFloatBuffer(numOfFloats);

        buffer.put(vertices);
        buffer.put(normals);
        buffer.put(texCoords);

        buffer.position(0);

        Gdx.gl30.glBindVertexArray(VAO);

        Gdx.gl.glBindBuffer(Gdx.gl.GL_ARRAY_BUFFER, VBO);
        Gdx.gl.glBufferData(Gdx.gl.GL_ARRAY_BUFFER, buffer.remaining()*4,buffer, Gdx.gl.GL_STATIC_DRAW);

        Gdx.gl.glBindBuffer(Gdx.gl.GL_ELEMENT_ARRAY_BUFFER, EBO);
        Gdx.gl.glBufferData(Gdx.gl.GL_ELEMENT_ARRAY_BUFFER, indices.remaining()*4, indices, Gdx.gl.GL_STATIC_DRAW);

        Gdx.gl.glVertexAttribPointer(0, 3, Gdx.gl.GL_FLOAT, false, 0, 0);
        vertices.position(0);
        Gdx.gl.glVertexAttribPointer(1,3,Gdx.gl.GL_FLOAT,false, 0, vertices.remaining()*4);
        vertices.position(0);
        normals.position(0);
        Gdx.gl.glVertexAttribPointer(2, 3, Gdx.gl.GL_FLOAT, false, 0, vertices.remaining()
        *4+normals.remaining()*4);
    }

    private void render(Shader shader){
        Gdx.gl.glEnableVertexAttribArray(0);
        Gdx.gl.glEnableVertexAttribArray(1);
        Gdx.gl.glEnableVertexAttribArray(2);

        Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE0);
        Gdx.gl.glBindTexture(Gdx.gl.GL_TEXTURE0, diffuseTexture);

        shader.use();
        Gdx.gl30.glBindVertexArray(VAO);
        Gdx.gl.glDrawElements(Gdx.gl.GL_TRIANGLES, indices.remaining(), Gdx.gl.GL_UNSIGNED_INT, 0);

        Gdx.gl30.glBindVertexArray(0);
    }

    @Override
    public void dispose() {
        Gdx.gl.glDeleteBuffer(VBO);
        Gdx.gl.glDeleteBuffer(EBO);
        Gdx.gl30.glDeleteVertexArrays(1, new int[VAO], 0);
    }
}
