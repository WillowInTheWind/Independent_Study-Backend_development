import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MxinfoComponent } from './mxinfo.component';

describe('MxinfoComponent', () => {
  let component: MxinfoComponent;
  let fixture: ComponentFixture<MxinfoComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MxinfoComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(MxinfoComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
